from twirp.asgi import TwirpASGIApp
# from twirp.exceptions import InvalidArgument

from helloworld import helloworld_pb2
from helloworld import helloworld_twirp


class GreeterService(object):
    def sayHello(self, context, request):
        print('context', context)
        print('request', repr(request))
        print('request.name', request.name)
        print('request.ver', request.ver)
        print('request.bloodType', request.bloodType)
        return helloworld_pb2.HelloReply(
            message="Hello {}!".format(request.name)
        )


service = helloworld_twirp.GreeterServer(service=GreeterService())
app = TwirpASGIApp()
app.add_service(service)
print(app._prefix)
print(service.prefix)
