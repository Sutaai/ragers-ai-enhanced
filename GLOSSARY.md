# Ragers

Ragers encrypts files in a git repository using the `age` format, so that teams can safely commit secrets to public repositories.

## Language

**Recipient**:
A public key (age X25519 or SSH) that can decrypt an encrypted file.
_Avoid_: user, member, key

**Alias**:
A named reference to a recipient, a group, or a recipients file. File entries address their recipients through aliases.
_Avoid_: recipient name, key name

**Group**:
A named set of recipient aliases. Encrypting to a group encrypts to every member of it.
_Avoid_: team, role, collection

**Recipients file**:
A file holding a list of public keys, one per line. Addressed through a `file:` alias.
_Avoid_: key file, recipients list

**File entry**:
A config record that maps a plaintext `src` path to an encrypted `out` path and declares which aliases may decrypt it.
_Avoid_: file, target, mapping

**Armor**:
The ASCII-armored encoding of the age format, as opposed to its compact binary form.
_Avoid_: ascii, pem

**Identity**:
A private key (age identity or SSH private key) used to decrypt files.
_Avoid_: secret, private key, credential

**Config file**:
The `.ragers.yaml` (or `.ragers.yml`) file that declares a repository's recipients and file entries.
_Avoid_: settings, manifest
