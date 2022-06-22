from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder.tests.factory import student_factory
from autograder_django_backend.utils.tests import get_client


@pytest.mark.django_db(transaction=True)
def test_get_students_of_professor():
    professor = Professor.objects.create(
        name="Professor Doe",
        email="test@test.com",
    )

    # Create a student
    student = student_factory()

    # Create a course
    course = Course.objects.create(
        course_id="COMP 2402",
        name="Abstract Data Types and Algorithms",
        description="Introduction to the design and implementation of abstract data types and to complexity analysis of data structures. Topics include: stacks, queues, lists, trees and graphs. Special attention is given to abstraction, interface specification and hierarchical design using an object-oriented programming language.",
        section="B",
        professor=professor,
    )

    # Add the student to the course
    student.courses.add(course)
    student.save()

    # Add the course to the professor
    course.professor = professor
    course.save()

    # Get the students of the professor
    students = professor.get_students()

    # Check that the student is in the list
    assert student in students


@pytest.mark.django_db(transaction=True)
def test_create_professor_api():
    client, user = get_client()

    response = client.post(
        "/api/professors/",
        {"name": "test_professor", "email": "test@test.com"},
        format="json",
    )

    assert response.status_code == 201

    assignment = Professor.objects.get(name="test_professor")

    assert assignment.name == "test_professor"
    assert assignment.email == "test@test.com"
