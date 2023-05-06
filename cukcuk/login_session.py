import hmac
import hashlib
import json
from datetime import datetime
import pytz
import requests

from .common import BASE_URL

class LoginSession:
    def __init__(self, *, app_id, domain, secrets):
        self.app_id = app_id
        self.domain = domain
        self.secrets = secrets
        self.__access_token = None
        self.login_time = datetime.now(pytz.UTC)
        self.login()

    @property
    def signature(self):
        message = json.dumps(self.__info_no_signature, separators=(",", ":"))
        signature = hmac.new(
            key=self.secrets.encode("utf-8"),
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
    def access_token(self):
        if self.__access_token == None:
            raise Exception("Must login before retrieving access token")
        return self.__access_token

    def login(self):
        url = f"{BASE_URL}/api/Account/Login"
        payload = self.__info_no_signature
        payload["SignatureInfo"] = self.signature
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
