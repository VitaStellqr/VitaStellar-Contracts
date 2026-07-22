#!/usr/bin/env python3
"""
Simple HTTP Server for VitaStellar Web DApp
"""
import http.server
import socketserver
import os
import sys

PORT = 8080
WEB_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'web')

os.chdir(WEB_DIR)

class CustomHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Cache-Control', 'no-store, no-cache, must-revalidate')
        super().end_headers()

if __name__ == '__main__':
    print(f"🌟 Launching VitaStellar Web DApp Portal at http://127.0.0.1:{PORT}")
    print(f"📁 Serving static files from: {WEB_DIR}")
    
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("127.0.0.1", PORT), CustomHandler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down server.")
            sys.exit(0)
