import argparse
import asyncio
import signal
import sys

import aiohttp

queue = asyncio.Queue()


async def start_client(url, loop):
    ws = await aiohttp.ClientSession().ws_connect(url, autoclose=False, autoping=False)

    def stdin_callback():
        line = sys.stdin.buffer.readline().decode('utf-8')
        if not line:
            loop.stop()
        else:
            # Queue.put is a coroutine, so you can't call it directly.
            asyncio.ensure_future(queue.put(ws.send_str(line)))

    loop.add_reader(sys.stdin, stdin_callback)

    async def dispatch():
        while True:
            msg = await ws.receive()
            if msg.type == aiohttp.WSMsgType.TEXT:
                print('Text: ', msg.data.strip())
            elif msg.type == aiohttp.WSMsgType.BINARY:
                print('Binary: ', msg.data)
            elif msg.type == aiohttp.WSMsgType.PING:
                await ws.pong()
            elif msg.type == aiohttp.WSMsgType.PONG:
                print('Pong received')
            else:
                if msg.type == aiohttp.WSMsgType.CLOSE:
                    await ws.close()
                elif msg.type == aiohttp.WSMsgType.ERROR:
                    print('Error during receive %s' % ws.exception())
                elif msg.type == aiohttp.WSMsgType.CLOSED:
                    pass
                break

    await dispatch()


async def tick():
    while True:
        await (await queue.get())


async def main(url, loop):
    await asyncio.wait([start_client(url, loop), tick()])


ARGS = argparse.ArgumentParser(
    description="websocket console client for wssrv.py example.")
ARGS.add_argument(
    '--host', action="store", dest='host',
    default='127.0.0.1', help='Host name')
ARGS.add_argument(
    '--port', action="store", dest='port',
    default=8080, type=int, help='Port number')

if __name__ == '__main__':
    args = ARGS.parse_args()
    if ':' in args.host:
        args.host, port = args.host.split(':', 1)
        args.port = int(port)

    url = 'http://{}:{}/ws/connect'.format(args.host, args.port)

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    signal.signal(signal.SIGINT, loop.stop)
    asyncio.Task(main(url, loop))
    loop.run_forever()