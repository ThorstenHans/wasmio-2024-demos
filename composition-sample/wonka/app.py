from spin_sdk import http, key_value
from spin_sdk.http import Request, Response

from verification.imports.verify import verify

from http_router import Router, exceptions
from urllib.parse import ParseResult, urlparse, parse_qs
import json

router = Router(trim_last_slash=True)

@router.route("/target", methods=["POST"])
def handle_inbound_webhook(uri: ParseResult, request: Request) -> Response:
    if uri.query == "handshake=true":
        return handle_handshake(request)
    else:
        return handle_invocation(request)

def handle_invocation(request: Request) -> Response:
    tag = request.headers.get("x-signature")
    print("WONKA: Received tag ",tag)
    tag = bytes(tag, 'utf-8')
    with key_value.open_default() as store:
        keydata = store.get("signing-key-data")
        print("WONKA: Loaded key data from key value store:", str(keydata))
        print("WONKA: Verifying integrity of payload received from ACME...")
        valid = verify(request.body, keydata, tag)
        print("-------------------")
        print("WONKA: Payload verification result:", bool(valid))
        print("-------------------")
    if valid == False:
        print("WONKA: Responding with HTTP 400")
        return Response(400, {"content-type": "text/plain"}, None)
    print("WONKA: Responding with HTTP 200")
    return Response(200, {"content-type": "text/plain"}, bytes("Received payload and verified integrity", "utf-8"))  

def handle_handshake(request: Request) -> Response:
    j = json.loads(request.body.decode('utf-8'))
    keyData = j["keyData"]
    print("WONKA: Received",keyData,"upon registering for webhooks with ACME product.")
    with key_value.open_default() as store:
        store.set("signing-key-data", bytes(keyData, "utf-8"))
        print("WONKA: Stored key data in key value store")
    return Response(200, {"content-type": "text/plain"}, None)  
    
class IncomingHandler(http.IncomingHandler):
    def handle_request(self, request: Request) -> Response:
        
        uri = urlparse(request.uri)
        try:
            handler = router(uri.path, request.method)
            return handler.target(uri, request)
        except exceptions.NotFoundError:  
            return Response(404, {}, None)