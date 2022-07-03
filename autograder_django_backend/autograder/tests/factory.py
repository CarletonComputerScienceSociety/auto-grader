from faker import Faker
from autograder.models import Student, Course, Professor


fake = Faker()

# Create a student
def student_factory(**kwargs):
    if "name" not in kwargs:
        kwargs["name"] = fake.name()
    if "email" not in kwargs:
        kwargs["email"] = fake.email()

    # Check that there are only 2 arguments in kwargs
    assert len(kwargs) == 2

    student = Student.objects.create(**kwargs)

    return student


# Create a professor
def professor_factory(**kwargs):
    if "name" not in kwargs:
        kwargs["name"] = fake.name()
    if "email" not in kwargs:
        kwargs["email"] = fake.email()

    # Check that there are only 2 arguments in kwargs
    assert len(kwargs) == 2

    professor = Professor.objects.create(**kwargs)

    return professor


# Create a course
def course_factory(**kwargs):
    if "course_id" not in kwargs:
        kwargs["course_id"] = fake.pystr()
    if "name" not in kwargs:
        kwargs["name"] = fake.name()
    if "description" not in kwargs:
        kwargs["description"] = fake.text()
    if "section" not in kwargs:
        kwargs["section"] = fake.random_letter()
    if "professor" not in kwargs:
        kwargs["professor"] = professor_factory()

    # Check that there are only 5 arguments in kwargs
    assert len(kwargs) == 5

    course = Course.objects.create(**kwargs)

    return course
