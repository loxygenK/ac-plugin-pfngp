# ac-plugin-pfngp
<a id="japanese"></a>
<a href="#english">English version is available.</a>

クリップボード内のデータを直接暗号化・復号するための、[autoclip](https://github.com/siketyan/autoclip)向けプラグインです。

今のところはテキストデータのみに対応しています。今後のリリースでファイルなどにも対応する予定です。

## 使い方
### 暗号化

1. 暗号化したいテキストデータに続けて、`/enc 対象ユーザ1 対象ユーザ2 ...` というように、
   **`/enc` に続けて受取人の名前をスペース区切りで入力します**。

2. `autoclip` を実行した状態でコピーします。

3. 対象ユーザが見つかった場合、**暗号化したデータがクリップボードに送られます**。<br />
   デスクトップ通知が送信されます。


### 復号

1. 復号したいデータをコピーします。<br />
   (編集は必要ありません。`-----BEGIN PGP MESSAGE-----` から `-----END PGP MESSAGE-----` をコピーしてください。)

2. 場合によっては秘密鍵のパスフレーズが要求されます。<br />
   復号のみに使われ、保存はされません。

3. 復元できた場合は復元結果がクリップボードに送られます。<br />
   デスクトップ通知が送信されます。

### 例

```bash
$ echo "this is an extremely secret message./enc loxy" | xsel -bi
$ xsel -bo
-----BEGIN PGP MESSAGE-----

hF4D185j84PEsgQSAQdAbGZiC5s8dR7umBu5mghERacNCMvy+8mF3F+xgmCG70Iw
Ck4ftzMmEyCLnw7wKLBBpGNQCNL/mL2pytQLKCEYC94c5YQz9tI1FTzsnXx65tOb
0l0BmqCMnbe0SgY5hVil4dIh95BOi2WPeYYX7KThiosQBKPr0VfdWFPfUA1aVEMV
X/kx/WEsqyolXCetXESv/yS5ml1exF0Ars5GnPRTygH9q2qihz4VDCSyBpUy0rM=
=+RBq
-----END PGP MESSAGE-----
```

```bash
$ cat <<EOF | gpg
-----BEGIN PGP MESSAGE-----

hF4D185j84PEsgQSAQdAbGZiC5s8dR7umBu5mghERacNCMvy+8mF3F+xgmCG70Iw
Ck4ftzMmEyCLnw7wKLBBpGNQCNL/mL2pytQLKCEYC94c5YQz9tI1FTzsnXx65tOb
0l0BmqCMnbe0SgY5hVil4dIh95BOi2WPeYYX7KThiosQBKPr0VfdWFPfUA1aVEMV
X/kx/WEsqyolXCetXESv/yS5ml1exF0Ars5GnPRTygH9q2qihz4VDCSyBpUy0rM=
=+RBq
-----END PGP MESSAGE-----
gpg: *警告*: コマンドが指定されていません。なにを意味しているのか当ててみます ...
gpg: 256-ビットECDH鍵, ID D7CE63F383C4B204, 日付2020-06-03に暗号化されました
      "loxygen.k <loxygen.k@gmail.com>"
this is an extremely secret message.
```

### 今後の実装予定

- 暗号化/復号化
  - [X] テキストデータ
  - [ ] バイナリデータ (ファイルへの出力/ファイルからの入力)

----

<a id="english"></a>
<a href="#japanese">日本語版が利用できます。</a>

A plugin for [autoclip](https://github.com/siketyan/autoclip) to encrypt/decrypt data in clipboard directly.ｈjjj

Currently only for text data. Encrypting/Decrypting binary data will be implemented in the future.

## How to use
### Encrypting

1. After the text you want to encrypt, type `/enc`, and enter recipients' name after it, separating by spaces,
   like `/enc recipients-1 recipients-2 ...`.

   Recipients' name does NOT have to be full, it's ok to partial one!

2. Execute `autoclip`, and copy the text modified in step 1.

3. If recipient's keys found successfully, **encrypted data is in your clipboard**.<br />
   The desktop notification is sent.

### Decrypting

1. Copy the data you want to decrypt.<br />
   (No modification needed, just copy from `-----BEGIN PGP MESSAGE-----` to `-----END PGP MESSAGE-----`)

2. If you set the passphrase to your secret key, you will be asked for passphrase of it.<br />
   It is only used for decryption, and will not be saved.

3. If the data decrypted successfully, **decrypted data is in your clipboard**.<br />
   The desktop notification is sent.

### Examples

```bash
$ echo "this is an extremely secret message./enc loxy" | xsel -bi
$ xsel -bo
-----BEGIN PGP MESSAGE-----

hF4D185j84PEsgQSAQdAbGZiC5s8dR7umBu5mghERacNCMvy+8mF3F+xgmCG70Iw
Ck4ftzMmEyCLnw7wKLBBpGNQCNL/mL2pytQLKCEYC94c5YQz9tI1FTzsnXx65tOb
0l0BmqCMnbe0SgY5hVil4dIh95BOi2WPeYYX7KThiosQBKPr0VfdWFPfUA1aVEMV
X/kx/WEsqyolXCetXESv/yS5ml1exF0Ars5GnPRTygH9q2qihz4VDCSyBpUy0rM=
=+RBq
-----END PGP MESSAGE-----
```

```bash
$ cat <<EOF | gpg
-----BEGIN PGP MESSAGE-----

hF4D185j84PEsgQSAQdAbGZiC5s8dR7umBu5mghERacNCMvy+8mF3F+xgmCG70Iw
Ck4ftzMmEyCLnw7wKLBBpGNQCNL/mL2pytQLKCEYC94c5YQz9tI1FTzsnXx65tOb
0l0BmqCMnbe0SgY5hVil4dIh95BOi2WPeYYX7KThiosQBKPr0VfdWFPfUA1aVEMV
X/kx/WEsqyolXCetXESv/yS5ml1exF0Ars5GnPRTygH9q2qihz4VDCSyBpUy0rM=
=+RBq
-----END PGP MESSAGE-----
gpg: WARNING: no command supplied.  Trying to guess what you mean ...
gpg: encrypted with 256-bit ECDH key, ID D7CE63F383C4B204, created 2020-06-03
      "loxygen.k <loxygen.k@gmail.com>"
this is an extremely secret message.
```

### Future implementation

- Encrypting/Decrypting
  - [X] Text data
  - [ ] Binary data (expecting file I/O)
