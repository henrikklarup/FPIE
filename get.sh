#!/usr/bin/env sh
set -e

BASE="https://github.com/hklsiteimprove/fpie/releases/download"
VERSION=$VERSION
TO="${TO:-.}"

UNAME_OUT="${UNAME_OUT:-$(uname -s)}"
case "${UNAME_OUT}" in
Linux*)
  MACHINE=Linux
  FILENAME=x86_64-unknown-linux-gnu.zip
  ;;
Darwin*)
  MACHINE=Mac
  FILENAME=x86_64-apple-darwin.zip
  ;;
CYGWIN*) MACHINE=Cygwin ;;
MINGW*) MACHINE=MinGw ;;
*)
  echo "Unknown machine: ${UNAME_OUT}"
  exit 1
  ;;
esac

if [ ${MACHINE} = "Linux" ]; then
  RELEASE_PRETTY_NAME="${RELEASE_PRETTY_NAME:-$(cat /etc/*-release | grep "PRETTY_NAME" | sed 's/PRETTY_NAME=//g')}"
  case "${RELEASE_PRETTY_NAME}" in
  *Alpine*)
    MACHINE=Alpine
    FILENAME=x86_64-unknown-linux-musl.zip
    ;;
  *Ubuntu*)
    MACHINE=Ubuntu
    FILENAME=x86_64-unknown-linux-gnu.zip
    ;;
  *Amazon*Linux*)
    MACHINE="Amazon Linux"
    FILENAME=x86_64-unknown-linux-musl.zip
    ;;
  *Linux*Mint*)
    MACHINE="Linux Mint"
    FILENAME=x86_64-unknown-linux-musl.zip
    ;;
  *)
    echo "Unknown Linux: ${RELEASE_PRETTY_NAME}"
    exit 1
    ;;
  esac
fi

URL="${BASE}/${VERSION}/${FILENAME}"
if [ -z "${DRYRUN}" ]; then
  curl -fL "${URL}" -o "${TO}/fpie.zip" && unzip -o "${TO}/fpie.zip" -d "${TO}" && rm -f "${TO}/fpie.zip" && chmod +x "${TO}/fpie"
else
  echo "Would download ${FILENAME} for ${MACHINE} from ${URL} to ${TO}"
fi
