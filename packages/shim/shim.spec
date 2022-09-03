%global debug_package %{nil}
%global __strip %{_bindir}/true

%global efidir /boot/efi/EFI/BOOT
%global commit 505cdb678b319fcf9a7fdee77c0f091b4147cbe5

Name: %{_cross_os}shim
Version: 15.6
Release: 1%{?dist}
Summary: UEFI shim loader
License: BSD-3-Clause
URL: https://github.com/rhboot/shim/
Source0: https://github.com/rhboot/shim/archive/%{version}/shim-%{version}.tar.gz
Source1: https://github.com/rhboot/gnu-efi/archive/refs/heads/shim-%{version}.tar.gz#/gnu-efi-shim-%{version}.tar.gz

%description
%{summary}.

%prep
%autosetup -n shim-%{version} -p1
%setup -T -D -n shim-%{version} -a 1
rmdir gnu-efi
mv gnu-efi-shim-%{version} gnu-efi

%global shim_make \
%make_build\\\
  ARCH="%{_cross_arch}"\\\
  CROSS_COMPILE="%{_cross_target}-"\\\
  COMMIT_ID="%{commit}"\\\
  RELEASE="%{release}"\\\
  DEFAULT_LOADER="%{_cross_grub_efi_image}"\\\
  DESTDIR="%{buildroot}"\\\
  EFIDIR="BOOT"\\\
%{nil}

%build
%shim_make

%install
%shim_make install-as-data
install -d %{buildroot}%{efidir}
find %{buildroot}%{_datadir} -name '%{_cross_shim_efi_image}' -exec \
  mv {} "%{buildroot}%{efidir}/%{_cross_boot_efi_image}" \;
rm -rf %{buildroot}%{_datadir}

%files
%license COPYRIGHT
%{_cross_attribution_file}
%dir %{efidir}
%{efidir}/%{_cross_boot_efi_image}

%changelog
