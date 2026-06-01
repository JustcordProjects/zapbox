#!/bin/sh
set -e

export PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
export LC_ALL=C

ZAP_VERSION=0.2.0
ZAP_URL="https://github.com/thezaplang/zap/releases/download/v$ZAP_VERSION/zap-$ZAP_VERSION-linux-x86_64.tar.gz"

#grep -q '^staff:' /etc/group || echo 'staff:x:50:' >> /etc/group

apt-get update
apt-get install -y --no-install-recommends \
    curl ca-certificates tar clang

mkdir -p /opt/zap
curl -sSL "$ZAP_URL" | tar -xzC /opt/zap --strip-components=1

cat << 'EOF' > /usr/local/bin/pipe
#!/bin/sh
file="$1"
shift
exec "$@" < "$file"
EOF
chmod +x /usr/local/bin/pipe
