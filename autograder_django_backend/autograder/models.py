from django.db import models


class Course(models.Model):
    course_id = models.CharField(max_length=10)
    name = models.CharField(blank=True, max_length=50)
    description = models.TextField(blank=True)
    section = models.CharField(max_length=5)
    professor = models.ForeignKey(
        "autograder.Professor", on_delete=models.CASCADE, related_name="courses"
    )

    def get_students_from_course(self):
        return list(Student.objects.filter(courses=self))


class Assignment(models.Model):
    name = models.CharField(max_length=100)
    description = models.TextField()
    due_date = models.DateTimeField()

    assignment_spec = models.TextField()
    # uploaded professor test cases as a file

    course = models.ForeignKey(
        "Course", on_delete=models.CASCADE, related_name="assignments"
    )


class Student(models.Model):
    name = models.CharField(max_length=100)
    email = models.EmailField()
    assignment = models.ForeignKey(
        Assignment, on_delete=models.CASCADE, related_name="students", null=True
    )

    courses = models.ManyToManyField(Course, related_name="students")

    def get_professors(self):
        return list(Professor.objects.filter(courses__in=self.courses.all()))


class Professor(models.Model):
    name = models.CharField(max_length=100)
    email = models.EmailField()

    def get_students(self):
        return list(Student.objects.filter(courses__professor=self).all())


# class Submission(models.Model):
#     # students uploaded code as a field

#     student = models.ForeignKey(Student, on_delete=models.CASCADE)
#     assignment = models.ForeignKey(Assignment, on_delete=models.CASCADE)


# class TestCase(models.Model):
#     question = models.ForeignKey(Question, on_delete=models.CASCADE)
