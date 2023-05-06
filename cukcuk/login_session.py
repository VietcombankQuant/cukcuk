import hmac
import hashlib
import json
from datetime import datetime
import pytz
import requests

from .common import BASE_URL
from .branch import Branch


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

    def get_all_branches(self) -> list[Branch]:
        url = f"{BASE_URL}/api/v1/branchs/all"
        resp = requests.get(
            url,
            headers=self.__auth_headers,
            params={"includeInactive": True}
        )
        if not resp.ok:
            raise Exception(
                f"Failed to fetch branch list with HTTP code {resp.status_code}"
            )
        try:
            message = json.loads(resp.text)
        except json.JSONDecodeError as err:
            raise Exception(
                f"Failed to decode response from url {url} with error {err.msg}"
            )

        if not message.get("Success", False):
            raise Exception(
                f"Failed to request branch list from {url} with error "
                f"HTTP code: {message.get('Code','')} - "
                f"ErrorType: {message.get('ErrorType','')} - "
                f"ErrorMessage: {message.get('ErrorMessage','')}"
            )

        branches = []
        records = message.get("Data", [{}])
        for record in records:
            branch = Branch()
            for key, value in record.items():
                if key in branch.__dict__:
                    branch.__dict__[key] = value

            branches.append(branch)

        return branches

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
