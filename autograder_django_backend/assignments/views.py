from rest_framework.viewsets import ViewSet
from rest_framework.response import Response
from .serializers import UploadSerializer
from rest_framework.parsers import FormParser, MultiPartParser

# ViewSets define the view behavior.


class UploadViewSet(ViewSet):
    serializer_class = UploadSerializer

    parser_classes = [MultiPartParser, FormParser]

    def list(self, request):
        return Response("GET API")

    def create(self, request):
        file_uploaded = request.data['file_uploaded']
        content_type = file_uploaded.content_type
        response = "POST API and you have uploaded a {} file".format(content_type)

        # Send request to scheduler

        return Response(response)
