const { spawn } = require('child_process');
const path = require('path');

export const config = {
  runtime: 'experimental-edge',
};

export default function handler(req, res) {
  const wasmedge = spawn(path.join(__dirname, 'wasmedge'), [
    path.join(__dirname, 'acmi_parser.so'),
  ]);

  let d = [];
  wasmedge.stdout.on('data', (data) => {
    d.push(data);
  });

  wasmedge.on('close', (code) => {
    let r = d.join('');
    let format = r.substring(0, 3);
    let buf = Buffer.from(r.substring(3), 'hex');

    res.setHeader('Content-Type', `image/${format}`);
    res.send(buf);
  });

  wasmedge.stdin.write(req.body);
  wasmedge.stdin.end('');
}
