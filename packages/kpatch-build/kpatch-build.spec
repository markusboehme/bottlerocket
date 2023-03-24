Name: %{_cross_os}kpatch-build
Version: 0.9.8
Release: 1%{?dist}
Summary: Kernel live patch build tools
License: GPL-2.0
URL: https://github.com/dynup/kpatch
Source0: https://github.com/dynup/kpatch/archive/refs/tags/v%{version}.tar.gz
BuildRequires: %{_cross_os}glibc-devel

%description
%{summary}.

%prep
%autosetup -n kpatch-%{version} -p1

%build
%make_build -C kpatch-build PREFIX=%{_cross_prefix}
%make_build -C kmod PREFIX=%{_cross_prefix} BUILDMOD=no

%install
%make_install -C kpatch-build PREFIX=%{_cross_prefix}
%make_install -C kmod PREFIX=%{_cross_prefix}

%files
%license COPYING
%{_cross_attribution_file}

%{_cross_bindir}/kpatch-build
%{_cross_datadir}/kpatch/patch
%{_cross_libexecdir}/kpatch/create-diff-object
%{_cross_libexecdir}/kpatch/create-klp-module
%{_cross_libexecdir}/kpatch/create-kpatch-module
%{_cross_libexecdir}/kpatch/kpatch-cc
