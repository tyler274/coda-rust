# A stand-in for libsecret's `secret-tool` in the agent-sandbox VM check,
# which has no desktop session to run a Secret Service. Items are files
# under `dir`, named by their `token` attribute; while `dir/locked` exists
# every call fails with a message, as a locked or absent keyring does. It
# keeps the exit codes the broker reads: 1 and silence for no match.
{ writeShellApplication, dir }:
writeShellApplication {
  name = "secret-tool";
  text = ''
    if [ -e ${dir}/locked ]; then
      echo "secret-tool: Cannot create an item in a locked collection" >&2
      exit 1
    fi
    op="$1"; shift
    if [ "$op" = store ]; then
      [ "$1" = --label ] && shift 2
    fi
    [ "$1" = service ] && [ "$2" = coda-sandbox ] && [ "$3" = token ] || {
      echo "secret-tool: unexpected attributes: $*" >&2
      exit 2
    }
    item="${dir}/$4"
    case "$op" in
      lookup) [ -f "$item" ] || exit 1; cat "$item" ;;
      store) cat > "$item" ;;
      clear) [ -f "$item" ] || exit 1; rm "$item" ;;
      *) echo "secret-tool: unknown operation $op" >&2; exit 2 ;;
    esac
  '';
}
