from flask import Flask
import dbm
import json
from loguru import logger

app = Flask(__name__)
db = dbm.open('secrets.dbm', 'c')

@app.route('/')
def hello_world():
    kv = {k.decode('utf-8'): db[k].decode('utf-8') for k in db.keys()}
    return json.dumps(kv)

@app.route('/get/<key>')
def get(key):
    return db[key]

@app.route('/set/<key>/<value>')
def set(key, value):
    old_value = ''
    if key in db:
        old_value = db[key]
    db[key] = value
    return old_value


