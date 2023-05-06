import hmac
import hashlib
import json
from datetime import datetime
import pytz
import requests

from .common import BASE_URL


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
