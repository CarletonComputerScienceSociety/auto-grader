from rest_framework import serializers

from .models import Assignment, Course, Professor, Student


class AssignmentSerializer(serializers.ModelSerializer):
    class Meta:
        model = Assignment
        fields = "__all__"


class CourseSerializer(serializers.ModelSerializer):
    class Meta:
        model = Course
        fields = "__all__"


class StudentSerializer(serializers.ModelSerializer):
    class Meta:
        model = Student
        fields = "__all__"


class ProfessorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Professor
        fields = "__all__"
