QEMU ?= qemu-system-aarch64
PORT ?= 2333

# qemu_image := ubuntu-18.04-server-arm64.iso
# qemu_efi := QEMU_EFI.fd
# qemu_args := \
#   -smp 4 -m 4G -accel tcg -nographic \
#   -M virt \
#   -cpu cortex-a57 \
#   -drive file=$(qemu_image) \
#   -net user,id=net -net nic \
#   -serial mon:stdio

qemu_args := \
  	-smp 4 -m 4G -nographic \
	-M virt \
	-cpu cortex-a57 \
	-bios QEMU_EFI.fd \
	-drive if=none,file=ubuntu-18.04-server-arm64.iso,id=cdrom,media=cdrom \
	-device virtio-scsi-device -device scsi-cd,drive=cdrom \
	-drive if=none,file=ubuntuimg.img,id=hd0 \
	-device virtio-blk-device,drive=hd0 \
	-net user,id=net -net nic \
    -serial mon:stdio


# $(qemu_image):
# 	wget https://old-releases.ubuntu.com/releases/18.04.3/$(qemu_image)

# $(qemu_efi):
# 	wget http://releases.linaro.org/components/kernel/uefi-linaro/16.02/release/qemu64/QEMU_EFI.fd

.ONESHELL:
image: $(qemu_image)
	cat >user-data <<EOF
	#cloud-config
	password: guest
	chpasswd: { expire: False }
	ssh_pwauth: True
	EOF
	cloud-localds user-data.img user-data
	qemu-img resize $(qemu_image) +64G
	$(QEMU) $(qemu_args) -drive file=user-data.img,format=raw

qemu: $(qemu_image)
	$(QEMU) $(qemu_args)

ssh:
	ssh -p $(PORT) ubuntu@localhost
