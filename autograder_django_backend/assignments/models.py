from django.db import models

class Assignment(models.Model):
    # TODO: Make IDs use UUIDs

    file = models.FileField(upload_to='assignments/')