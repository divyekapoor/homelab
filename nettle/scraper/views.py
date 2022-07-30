# -*- coding: utf-8 -*-
from __future__ import unicode_literals
from http.client import HTTPResponse

from django.shortcuts import render

# Create your views here.
def index(request):
    return HTTPResponse('Hello world! Scraper says hi!')