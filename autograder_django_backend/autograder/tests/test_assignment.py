from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder_django_backend.autograder_django_backend.utils.tests import get_client


@pytest.mark.django_db(transaction=True)
def test_create_assignment_api():
    client, user = get_client()

    client.post(
        '/api/assignments/',
    )