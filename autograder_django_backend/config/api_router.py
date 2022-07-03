from django.conf import settings
from rest_framework.routers import DefaultRouter, SimpleRouter
from autograder.views import AssignmentViewSet

from autograder_django_backend.users.api.views import UserViewSet

if settings.DEBUG:
    router = DefaultRouter()
else:
    router = SimpleRouter()

router.register("users", UserViewSet)
router.register("assignments", AssignmentViewSet)


app_name = "api"
urlpatterns = router.urls
