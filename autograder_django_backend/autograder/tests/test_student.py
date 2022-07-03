from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student
from autograder.tests.factory import course_factory, professor_factory, student_factory


@pytest.mark.django_db(transaction=True)
def test_student_get_all_professors():
    student_one = student_factory(
        name="Student One",
        email="studentone@test.com",
    )

    professor_one = professor_factory()
    professor_two = professor_factory()

    # Create a course
    course_one = course_factory(
        professor=professor_one,
    )

    course_two = course_factory(
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
