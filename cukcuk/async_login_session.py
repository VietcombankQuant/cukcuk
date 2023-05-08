from datetime import datetime
import aiohttp
import asyncio

from cukcuk.branch import Branch
from cukcuk.invoice import Invoice

from .branch import Branch
from .login_session import LoginSession
from .common import BASE_URL, handle_response_async


class AsyncLoginSession(LoginSession):
    def __init__(self, *, app_id, domain, secret_key):
        super().__init__(app_id=app_id, domain=domain, secret_key=secret_key)

    def api_client(self) -> aiohttp.ClientSession:
        return aiohttp.ClientSession(
            base_url=BASE_URL,
            headers=self._auth_headers,
        )

    async def get_all_branches(self, details=True) -> list[Branch]:
        url = "/api/v1/branchs/all"
        async with self.api_client() as client:
            resp = await client.get(url, params={"includeInactive": "true"})
            records = await handle_response_async(resp)
            tasks = []
            for record in records:
                branch_id = record.get("Id")
                if branch_id != None:
                    task = self.__get_branch_detail(client, branch_id)
                    tasks.append(task)
            branches = await asyncio.gather(*tasks)
            return branches

    async def get_invoices(self, branch: Branch, last_sync_date: datetime = None) -> list[Branch]:
        if last_sync_date == None:
            last_sync_date = datetime.today()

        all_invoices = []
        page = 1
        while True:
            invoices = await self.get_invoice_paging(branch, page=page, last_sync_date=last_sync_date)
            if len(invoices) == 0:
                break
            all_invoices.extend(invoices)
            page += 1

        return all_invoices

    async def get_invoice_paging(self, branch: Branch, page: int, limit: int = 100, last_sync_date: datetime = None) -> list[Branch]:
        url = "/api/v1/sainvoices/paging"
        if last_sync_date == None:
            last_sync_date = datetime.today()
        payload = {
            "Page": page,
            "Limit": limit,
            "BranchId": branch.Id,
            "LastSyncDate": last_sync_date.strftime("%Y-%m-%dT%H:%M:%SZ"),
            "HaveCustomer": "true",
        }

        async with self.api_client() as client:
            resp = await client.post(url, json=payload)
            records = await handle_response_async(resp)
            tasks = []
            for record in records:
                invoice_ref = record.get("RefId")
                if invoice_ref != None:
                    task = self.get_invoice(invoice_ref)
                    tasks.append(task)

        branches = await asyncio.gather(*tasks)
        return branches

    async def get_invoice(self, invoice_ref: str) -> Invoice:
        async with self.api_client() as client:
            # get basic invoice info
            url = f"/api/v1/sainvoices/{invoice_ref}"
            resp = await client.get(url)
            record = await handle_response_async(resp)

            # get detail info of invoice
            url = f"/api/v1/sainvoices/detail/{invoice_ref}"
            resp = await client.get(url)
            details = await handle_response_async(resp)

        record.update(details)
        invoice = Invoice.deserialize(record)
        return invoice

    async def __get_branch_detail(self, client: aiohttp.ClientSession, branch_id: str) -> Branch:
        url = f"/api/v1/branchs/setting/{branch_id}"
        resp = await client.get(url)
        record = await handle_response_async(resp)
        branch = Branch.deserialize(record)
        return branch
