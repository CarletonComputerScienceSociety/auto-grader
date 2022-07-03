from faker import Faker
from autograder.models import Assignment, Student


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
    if "description" not in kwargs:
        kwargs["description"] = fake.text()
    if "due_date" not in kwargs:
        kwargs["due_date"] = fake.date_time_between(start_date="-1y", end_date="now")
    if "assignment_spec" not in kwargs:
        kwargs["assignment_spec"] = fake.text()

    # Check that there are only 4 arguments in kwargs
    assert len(kwargs) == 4

    student = Assignment.objects.create(**kwargs)

    return student
