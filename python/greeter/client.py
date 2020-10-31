from twirp.context import Context
from twirp.exceptions import TwirpServerException

import sys
from pathlib import Path
sys.path.append(str(Path(__file__).parent.parent))

from helloworld import helloworld_pb2
from helloworld import helloworld_twirp

client = helloworld_twirp.GreeterClient("http://localhost:5000")

try:
    response = client.sayHello(ctx=Context(),
                               request=helloworld_pb2.HelloRequest(
        name='you', ver=123, bloodType="B"))
    print(response)
except TwirpServerException as e:
    print(e.code, e.message, e.meta, e.to_dict())
