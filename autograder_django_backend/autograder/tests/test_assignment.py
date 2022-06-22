from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder_django_backend.utils.tests import get_client

# Test creating an assignment through the API
@pytest.mark.django_db(transaction=True)
def test_create_assignment_api():
    client, user = get_client()

    professor = Professor.objects.create(name="test_professor", email="test@test.com")

    course = Course.objects.create(
        course_id="test_course",
        name="test_course",
        description="test_course",
        section="test_course",
        professor=professor,
    )

    response = client.post(
        "/api/assignments/",
        {
            "name": "test_assignment",
            "description": "test_description",
            "due_date": "2020-01-01T00:00:00Z",
            "assignment_spec": "test_assignment_spec",
            "course": course.id,
        },
        format="json",
    )

    assert response.status_code == 201

    assignment = Assignment.objects.get(name="test_assignment")

    assert assignment.name == "test_assignment"
    assert assignment.description == "test_description"
