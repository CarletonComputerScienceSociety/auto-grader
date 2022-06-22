from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder.tests.factory import student_factory


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
