from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder.tests.factory import student_factory
from autograder_django_backend.utils.tests import get_client


@pytest.mark.django_db(transaction=True)
def test_student_get_all_professors():
    student_one = student_factory(
        name="Student One",
        email="studentone@test.com",
    )

    professor_one = Professor.objects.create(
        name="Professor One",
        email="professorone@test.com",
    )

    professor_two = Professor.objects.create(
        name="Professor Two",
        email="professortwo@test.com",
    )

    # Create a course
    course_one = Course.objects.create(
        course_id="COMP 2402",
        name="Abstract Data Types and Algorithms",
        description="Introduction to the design and implementation of abstract data types and to complexity analysis of data structures. Topics include: stacks, queues, lists, trees and graphs. Special attention is given to abstraction, interface specification and hierarchical design using an object-oriented programming language.",
        section="B",
        professor=professor_one,
    )

    course_two = Course.objects.create(
        course_id="COMP 2406",
        name="Fundamentals of Web Applications",
        description="Introduction to Internet application development; emphasis on computer science fundamentals of technologies underlying web applications. Topics include: scripting and functional languages, language-based virtual machines, database query languages, remote procedure calls over the Internet, and performance and security concerns in modern distributed applications.",
        section="B",
        professor=professor_two,
    )

    # Save Student
    student_one.courses.add(course_one)
    student_one.courses.add(course_two)
    student_one.save()

    # Save professors
    course_one.professor = professor_one
    course_two.professor = professor_two
    professor_one.save()
    professor_two.save()

    # Get all professors
    list_of_professors = student_one.get_professors()

    assert professor_one in list_of_professors
    assert professor_two in list_of_professors


@pytest.mark.django_db(transaction=True)
def test_create_student_api():
    client, user = get_client()

    response = client.post(
        "/api/students/",
        {
            "name": "test_student",
            "email": "test@test.com",
        },
        format="json",
    )

    print(response.data)

    assert response.status_code == 201

    assignment = Student.objects.get(name="test_student")

    assert assignment.name == "test_student"
    assert assignment.email == "test@test.com"
