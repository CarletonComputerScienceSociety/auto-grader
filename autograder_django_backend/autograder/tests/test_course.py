from django.test import TestCase
import pytest
from autograder.models import Course, Assignment, Professor, Student

@pytest.mark.django_db(transaction=True)
def test_create_course():
    professor = Professor.objects.create(
        name="Professor Doe",
        email="test@test.com",
    )

    course = Course.objects.create(
        course_id="COMP 2402",
        name="Abstract Data Types and Algorithms",
        description="Introduction to the design and implementation of abstract data types and to complexity analysis of data structures. Topics include: stacks, queues, lists, trees and graphs. Special attention is given to abstraction, interface specification and hierarchical design using an object-oriented programming language.",
        section="B",
        professor=professor,
    )

    assert course.course_id == "COMP 2402"
    assert course.name == "Abstract Data Types and Algorithms"

    test_course_two = Course.objects.create(
        course_id="COMP 2406",
        name="Fundamentals of Web Applications",
        description="	Introduction to Internet application development; emphasis on computer science fundamentals of technologies underlying web applications. Topics include: scripting and functional languages, language-based virtual machines, database query languages, remote procedure calls over the Internet, and performance and security concerns in modern distributed applications.",
        section="A",
        professor=professor,
    )

    assert test_course_two.course_id == "COMP 2406"
    assert test_course_two.name == "Fundamentals of Web Applications"


@pytest.mark.django_db(transaction=True)
def test_get_students_of_course():

    # Creating a professor
    example_professor = Professor.objects.create(
        name="Professor Doe",
        email="test@test.com",
    )

    # Creating three students
    student_one = Student.objects.create(
        name="Student One",
        email="studentone@test.com",
    )

    student_two = Student.objects.create(
        name="Student Two",
        email="studenttwo@test.com",
    )

    student_three = Student.objects.create(
        name="Student Three",
        email="studentthree@test.com",
    )

    # Create a course
    example_course = Course.objects.create(
        course_id="COMP 2406",
        name="Fundamentals of Web Applications",
        description="Introduction to Internet application development; emphasis on computer science fundamentals of technologies underlying web applications. Topics include: scripting and functional languages, language-based virtual machines, database query languages, remote procedure calls over the Internet, and performance and security concerns in modern distributed applications.",
        section="A",
        professor=example_professor,
    )

    student_one.courses.add(example_course)
    student_two.courses.add(example_course)
    student_three.courses.add(example_course)

    list_of_students = example_course.get_students_from_course()

    assert student_one in list_of_students
    assert student_two in list_of_students
    assert student_three in list_of_students