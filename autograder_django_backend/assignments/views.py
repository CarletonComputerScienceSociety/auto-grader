from rest_framework.viewsets import ViewSet
from rest_framework.response import Response
from .serializers import UploadSerializer
from rest_framework.parsers import FormParser, MultiPartParser
import requests

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

        # Make sure it's a zip file
        if content_type != 'application/zip':
            response = "Please upload a zip file"
        else:
            # Send request to scheduler
            url = 'http://localhost:4000/add_job'
            files = {'file_uploaded': file_uploaded}
            r = requests.post(url, files=files)
            response = r.text

            print(response)



        return Response(response)
