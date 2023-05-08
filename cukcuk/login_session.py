import hmac
import hashlib
import json
from datetime import datetime
import pytz
import requests

from .common import BASE_URL, handle_response
from .branch import Branch
from .invoice import Invoice


class LoginSession:
    def __init__(self, *, app_id, domain, secret_key):
        self.app_id = app_id
        self.domain = domain
        self.secret_key = secret_key
        self.login_time = datetime.now(pytz.UTC)
        self.__access_token = None
        self.__login()

    @classmethod
    def from_json(cls, file_path: str):
        with open(file_path, "r") as f:
            json_data = f.read()
            kwargs = json.loads(json_data)
            return cls(**kwargs)

    @property
    def access_token(self):
        if self.__access_token == None:
            raise Exception("Must login before retrieving access token")
        return self.__access_token

    @property
    def api_client(self) -> requests.Session:
        if "__api_client" not in self.__dict__:
            self.__api_client = requests.Session()
            self.__api_client.headers.update(self._auth_headers)
        return self.__api_client

    def get_all_branches(self, details=True) -> list[Branch]:
        url = f"{BASE_URL}/api/v1/branchs/all"
        resp = self.api_client.get(url, params={"includeInactive": True})

        records = handle_response(resp)
        branches = []
        for record in records:
            branch_id = record.get("Id", None)
            if branch_id != None:
                branch = self.__get_branch_detail(branch_id)
                branches.append(branch)

        return branches

    def get_invoice_paging(self, branch: Branch, page: int, limit: int = 100, last_sync_date: datetime = None) -> list[Branch]:
        url = f"{BASE_URL}/api/v1/sainvoices/paging"
        if last_sync_date == None:
            last_sync_date = datetime.today()
        payload = {
            "Page": page,
            "Limit": limit,
            "BranchId": branch.Id,
            "LastSyncDate": last_sync_date.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "HaveCustomer": True,
        }
        resp = self.api_client.post(url, json=payload)
        records = handle_response(resp)

        invoices = []
        for record in records:
            invoice_ref = record.get("RefId", "")
            invoice = self.get_invoice(invoice_ref)
            invoices.append(invoice)

        return invoices

    def get_invoice(self, invoice_ref: str) -> Invoice:
        # get basic invoice info
        url = f"{BASE_URL}/api/v1/sainvoices/{invoice_ref}"
        resp = self.api_client.get(url)
        record = handle_response(resp)

        # get detail info of invoice
        url = f"{BASE_URL}/api/v1/sainvoices/detail/{invoice_ref}"
        resp = self.api_client.get(url)
        details = handle_response(resp)
        record.update(details)
        invoice = Invoice.deserialize(record)
        return invoice

    def __get_branch_detail(self, branch_id: str) -> Branch:
        url = f"{BASE_URL}/api/v1/branchs/setting/{branch_id}"
        resp = self.api_client.get(url)

        record = handle_response(resp)
        branch = Branch.deserialize(record)
        return branch

    @property
    def __signature(self):
        message = json.dumps(self.__info_no_signature, separators=(",", ":"))
        signature = hmac.new(
            key=self.secret_key.encode("utf-8"),
            msg=message.encode("utf-8"),
            digestmod=hashlib.sha256
        )
        return signature.hexdigest()

    @property
    def __info_no_signature(self):
        return {
            "AppID": self.app_id,
            "Domain": self.domain,
            "LoginTime": self.login_time.strftime("%Y-%m-%dT%H:%M:%SZ")
        }

    @property
    def _auth_headers(self):
        return {
            "CompanyCode": self.domain,
            "Authorization": f"Bearer {self.access_token}"
        }

    def __login(self):
        url = f"{BASE_URL}/api/Account/Login"
        payload = self.__info_no_signature
        payload["SignatureInfo"] = self.__signature
        resp = requests.post(url, json=payload)
        if not resp.ok:
            raise Exception(
                f"Failed to login with status {resp.status_code}. Check your login info"
            )

        content = json.loads(resp.text)
        if not content.get("Success", False):
            raise Exception(
                f'Failed to login with error message {content["ErrorMessage"]}'
            )

        self.__access_token = content["Data"]["AccessToken"]
