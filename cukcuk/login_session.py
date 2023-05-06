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

    @property
    def access_token(self):
        if self.__access_token == None:
            raise Exception("Must login before retrieving access token")
        return self.__access_token

    def get_all_branches(self, details=True) -> list[Branch]:
        url = f"{BASE_URL}/api/v1/branchs/all"
        resp = requests.get(
            url,
            headers=self.__auth_headers,
            params={"includeInactive": True}
        )

        records = handle_response(resp)
        branches = []
        for record in records:
            branch_id = record.get("Id", None)
            if branch_id != None:
                branch = self.__get_branch_detail(branch_id)
                branches.append(branch)

        return branches

    def get_invoice_list(self, branch: Branch, page: int, limit: int = 100, last_sync_date: datetime = None) -> list[Branch]:
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
        resp = requests.post(url, headers=self.__auth_headers, json=payload)
        message = handle_response(resp)
        return message

    def get_invoice(self, invoice_ref: str) -> Invoice:
        url = f"{BASE_URL}/api/v1/sainvoices/{invoice_ref}"
        resp = requests.get(url, headers=self.__auth_headers)
        message = handle_response(resp)

    def __get_branch_detail(self, branch_id: str) -> Branch:
        url = f"{BASE_URL}/api/v1/branchs/setting/{branch_id}"
        resp = requests.get(url, headers=self.__auth_headers)

        record = handle_response(resp)
        branch = Branch()
        for key, value in record.items():
            if key in branch.column_names():
                branch.__dict__[key] = value

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
    def __auth_headers(self):
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
