from faker import Faker
from autograder.models import Student


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

def assignment_factory(**kwargs):
    if "name" not in kwargs:
        kwargs["name"] = fake.name()
    if "email" not in kwargs:
        kwargs["email"] = fake.email()

    # Check that there are only 2 arguments in kwargs
    assert len(kwargs) == 2

    student = Student.objects.create(**kwargs)

    return student
