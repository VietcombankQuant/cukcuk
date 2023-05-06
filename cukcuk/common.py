import requests
import json
from http.client import responses as http_responses
from typing import Union

BASE_URL = "https://graphapi.cukcuk.vn"


def handle_response(resp: requests.Response) -> Union[dict, list]:
    if not resp.ok:
        raise Exception(
            f"Failed to send {resp.request.method} request to {resp.url} "
            f"with HTTP code {resp.status_code} - {http_responses[resp.status_code]}"
        )

    try:
        message = json.loads(resp.text)
    except json.JSONDecodeError as err:
        raise Exception(
            f"Failed to decode response from url {resp.url} with error {err.msg}"
        )

    if not message.get("Success", False):
        raise Exception(
            f"Failed to request branch list from {resp.url} with error "
            f"HTTP code: {message.get('Code','')} - "
            f"ErrorType: {message.get('ErrorType','')} - "
            f"ErrorMessage: {message.get('ErrorMessage','')}"
        )

    data = message["Data"]
    return data
