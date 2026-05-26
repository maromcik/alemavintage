#podman login cerit.io
podman build -t 192.168.0.10:5000/alemavintage-image .
#podman tag 192.168.1.10:5000/alemavintage-image cerit.io/roman_alexander_mariancik/alemavintage-image
#podman push cerit.io/roman_alexander_mariancik/alemavintage-image:latest
podman push 192.168.0.10:5000/alemavintage-image
ssh roman@server systemctl --user restart alemavintage.service
ssh roman@hp systemctl --user restart alemavintage.service
#kubectl apply -f kubernetes/alemavintage -n mariancik-ns