from flask import Flask, request
import dbm
import json
import secrets
from loguru import logger
from apscheduler.schedulers.background import BackgroundScheduler
from werkzeug.exceptions import HTTPException
from IPy import IP

app = Flask(__name__)
db = dbm.open('secrets.dbm', 'c')  # secret_key: secret_value
tokens = dbm.open('tokens.dbm', 'c')  # auth_token: ip_address
admins = dbm.open('admins.dbm', 'c')  # admin_name: admin_token (rotating)

# Security hole! Delete before prod.
# Register admins here. The keys will get rotated for all non-test keys.
# Comment out admins['test'] to secure the server.
admins['test'] = 'test'
admins['divye'] = 'foobar'

scheduler = BackgroundScheduler()

@scheduler.scheduled_job('interval', minutes=2)
def admin_rotator():
    logger.info('admin_rotator called.')
    for k in admins.keys():
        if k != 'test':
            admins[k] = secrets.token_urlsafe(8)
        logger.info('Rotating Admin Key: {}:{}', k, admins[k])

class AuthenticationError(HTTPException):
    code = 403
    description = 'Authentication Error - use the ?id= parameter'

class AdminAuthenticationError(HTTPException):
    code = 403
    description = 'Admin Authentication Error - use the ?id= parameter'

def verify_machine_token_or_throw():
    if 'test' in admins:
        return
    id_ = request.args.get('id')
    if id_ is not None and id_ in tokens:
        logger.info('Verified Machine {}:{} at IP: {}', id_, tokens[id_], request.remote_addr)
        return
    if id_ is not None and id_ in admins:
        logger.info('Verified Machine {}:{} at IP: {}', id_, admins[id_], request.remote_addr)
        return
    raise AuthenticationError()

def verify_admin_token_or_throw():
    if 'test' in admins:
        return
    id_ = request.args.get('id')
    if id_ is not None and id_ in admins:
        logger.info('Verified Admin {}:{} at IP: {}', id_, admins[id_], request.remote_addr)
        return
    raise AdminAuthenticationError()

@app.route('/')
def default():
    logger.info('Url map: {}', app.url_map)
    return str(app.url_map)

@app.route('/secrets')
def get_all():
    verify_machine_token_or_throw()
    kv = {k.decode('utf-8'): db[k].decode('utf-8') for k in db.keys()}
    return json.dumps(kv)


@app.route('/secrets/get/<key>')
def secrets_get(key):
    verify_machine_token_or_throw()
    return db[key]

@app.route('/secrets/set/<key>/<value>')
def secrets_set(key, value):
    verify_machine_token_or_throw()
    old_value = ''
    if key in db:
        old_value = db[key]
    db[key] = value
    return old_value

@app.route('/admins')
def admins_get():
    verify_admin_token_or_throw()
    return {k.decode('utf-8'): admins[k].decode('utf-8') for k in admins.keys()}

@app.route('/machines')
def machines_get():
    verify_admin_token_or_throw()
    return {k.decode('utf-8'): tokens[k].decode('utf-8') for k in tokens.keys()}

@app.route('/machines/register')
def machines_register():
    verify_admin_token_or_throw()
    token = secrets.token_urlsafe(8)
    tokens[token] = str(request.remote_addr) + '/' + str(IP(request.remote_addr).reverseName())
    return token

@app.route('/machines/unregister/<token>')
def machines_unregister(token):
    verify_admin_token_or_throw()
    del machines[token]
    return {}

@app.cli.command('add-admin-user')
@click.argument('name')
def add_admin_user(name):
    admins[name] = 'CLI'

if __name__ == '__main__':
    app.run(use_reloader=False, host='0.0.0.0', port=59999, debug=True)
