from spin_sdk import http, key_value
from spin_sdk.http import Request, Response

from consumer.imports.verify import verify

from http_router import Router, exceptions
from urllib.parse import ParseResult, urlparse, parse_qs
import json

router = Router(trim_last_slash=True)

@router.route("/target", methods=["POST"])
def handle_post(uri: ParseResult, request: Request) -> Response:
    if uri.query == "handshake=true":
        return handle_verification(request)
    else:
        return handle_invocation(request)

def handle_verification(request: Request) -> Response:
    j = json.loads(request.body.decode('utf-8'))
    keyData = j["keyData"]
    print(keyData)
    with key_value.open_default() as store:
        store.set("signing-key-data", bytes(keyData, "utf-8"))

    return Response(200, {"content-type": "text/plain"}, None)

def handle_invocation(request: Request) -> Response:
    tag = request.headers.get("x-signature")
    print("Received tag via HTTP Header:", tag)
    tag = bytes(tag, 'utf-8')
    with key_value.open_default() as store:
        keydata = store.get("signing-key-data")
        print("Found key data:", keydata)
        valid = verify(request.body, keydata, tag)
        print("Validation returned:", valid)
    if valid == False:
        return Response(400, {"content-type": "text/plain"}, None)
    return Response(200, {"content-type": "text/plain"}, None)    
    
    

class IncomingHandler(http.IncomingHandler):
    def handle_request(self, request: Request) -> Response:
        
        # res = verify(bytes("", 'utf-8'), bytes("", 'utf-8'), bytes("", 'utf-8'))
        uri = urlparse(request.uri)
        try:
            handler = router(uri.path, request.method)
            return handler.target(uri, request)
        except exceptions.NotFoundError:  
            return Response(404, {}, None)