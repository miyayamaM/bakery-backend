# データベースの更新

1.  マイグレーションファイルの作成
2.  マイグレーションの実行

  ```shell

  $ DATABASE_URL="postgres://postgres:postgres@127.0.0.1/bakery_backend" sea-orm-cli migrate up

  // ロールバック
  $ DATABASE_URL="postgres://postgres:postgres@127.0.0.1/bakery_backend" sea-orm-cli migrate down

  // すべてロールバックして、1から実行
  $ DATABASE_URL="postgres://postgres:postgres@127.0.0.1/bakery_backend" sea-orm-cli migrate refresh
  ```

3.  Entityの生成

  ```shell

  $ sea-orm-cli generate entity \
    -u postgres://postgres:postgres@127.0.0.1/bakery_backend \
    -o src/entities

  ```
