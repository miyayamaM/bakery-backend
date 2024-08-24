# データベースの更新

1.  マイグレーションファイルの作成
2.  マイグレーションの実行

  ```shell

  $ DATABASE_URL="postgres://postgres:postgres@127.0.0.1/bakery_backend" sea-orm-cli migrate refresh

  ```
