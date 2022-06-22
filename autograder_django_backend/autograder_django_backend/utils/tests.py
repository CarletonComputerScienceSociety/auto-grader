from rest_framework.test import APIClient
from autograder_django_backend.autograder_django_backend.users.models import User


def get_client():
    """
    Returns a Django test client.
    """

    client = APIClient()

    test_user = User.objects.create_user(username='test_user', password='test_password')

    client.login(username='test_user', password='test_password')

    return (client, test_user)
