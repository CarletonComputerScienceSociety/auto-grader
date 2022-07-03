from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder.tests.factory import course_factory, student_factory, professor_factory


@pytest.mark.django_db(transaction=True)
def test_get_students_of_professor():
    professor = professor_factory()

    # Create a student
    student = student_factory()

    # Create a course
    course = course_factory(
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
