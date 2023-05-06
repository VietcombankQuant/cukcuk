from sqlalchemy.sql.schema import Table as SqlTable
from sqlalchemy import Engine as SqlEngine, Connection as SqlConnection
from sqlalchemy.orm import DeclarativeBase
import requests
import json
from http.client import responses as http_responses
from typing import Union

BASE_URL = "https://graphapi.cukcuk.vn"


class SqlTableBase(DeclarativeBase):
    pass


class SqlTableMixin:
    def __init__(self):
        super().__init__()
        for column in self.column_names():
            self.__dict__[column] = None

    @classmethod
    def this_table(self) -> SqlTable:
        return self.metadata.tables[self.__tablename__]

    @classmethod
    def column_names(cls) -> list[str]:
        return [column.name for column in cls.this_table().columns]

    @classmethod
    def create_table(cls, engine: Union[SqlEngine, SqlConnection]):
        cls.this_table.create(engine)

    def __repr__(self) -> str:
        fields = {}
        for column in self.column_names():
            fields[column] = self.__dict__.get(column, None)
        result = json.dumps(fields)
        return result


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
