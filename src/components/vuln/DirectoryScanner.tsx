import { useState, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { IconFolder, IconSearch, IconWarning, IconLightbulb } from "../icons";

interface DirectoryScannerProps {
  onScan: (path: string) => Promise<void>;
  scanning: boolean;
  error: string | null;
}

export function DirectoryScanner({ onScan, scanning, error }: DirectoryScannerProps) {
  const [selectedPath, setSelectedPath] = useState<string>("");

  const handleSelectDirectory = useCallback(async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "スキャンするディレクトリを選択",
      });

      if (selected && typeof selected === "string") {
        setSelectedPath(selected);
      }
    } catch (e) {
      console.error("Failed to open directory dialog:", e);
    }
  }, []);

  const handleScan = useCallback(async () => {
    if (selectedPath) {
      await onScan(selectedPath);
    }
  }, [selectedPath, onScan]);

  const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setSelectedPath(e.target.value);
  }, []);

  return (
    <div className="directory-scanner">
      <h3 className="scanner-title">
        <IconSearch size={20} className="inline-icon" />
        ディレクトリスキャン
      </h3>
      
      <p className="scanner-description">
        プロジェクトディレクトリを選択して、依存関係の脆弱性をスキャンします。
        package-lock.json、Cargo.lock、requirements.txt、go.sum などのロックファイルを自動検出します。
      </p>

      <div className="scanner-input-group">
        <input
          type="text"
          value={selectedPath}
          onChange={handleInputChange}
          placeholder="ディレクトリパスを入力または選択..."
          className="scanner-path-input"
          disabled={scanning}
        />
        <button
          type="button"
          onClick={handleSelectDirectory}
          disabled={scanning}
          className="btn-secondary"
          title="ディレクトリを選択"
        >
          <IconFolder size={16} />
          選択
        </button>
        <button
          type="button"
          onClick={handleScan}
          disabled={!selectedPath || scanning}
          className="btn-primary"
        >
          {scanning ? (
            <>
              <span className="loading-spinner small" />
              スキャン中...
            </>
          ) : (
            <>
              <IconSearch size={16} />
              スキャン実行
            </>
          )}
        </button>
      </div>

      {error && (
        <div className="scanner-error">
          <IconWarning size={16} className="inline-icon" />
          {error}
        </div>
      )}

      <div className="scanner-supported">
        <h4>対応パッケージマネージャー</h4>
        <ul className="supported-list">
          <li>
            <span className="ecosystem-tag npm">npm/yarn</span>
            package-lock.json, pnpm-lock.yaml, yarn.lock
          </li>
          <li>
            <span className="ecosystem-tag cargo">Cargo</span>
            Cargo.lock
          </li>
          <li>
            <span className="ecosystem-tag pip">pip</span>
            requirements.txt, poetry.lock, Pipfile.lock
            <span 
              className="ecosystem-tip" 
              data-tip="requirements.txt は生成方法により間接依存が含まれない場合があります。&#10;&#10;• pip freeze: 間接依存を含む（推奨）&#10;• 手動作成: 直接依存のみ&#10;&#10;包括的なスキャンには poetry.lock や Pipfile.lock を推奨します。"
            >
              <IconLightbulb size={14} className="tip-icon" />
            </span>
          </li>
          <li>
            <span className="ecosystem-tag go">Go</span>
            go.sum
          </li>
          <li>
            <span className="ecosystem-tag maven">Maven/Gradle</span>
            pom.xml, gradle.lockfile
          </li>
          <li>
            <span className="ecosystem-tag nuget">NuGet</span>
            packages.lock.json
          </li>
          <li>
            <span className="ecosystem-tag ruby">RubyGems</span>
            Gemfile.lock
          </li>
          <li>
            <span className="ecosystem-tag php">Composer</span>
            composer.lock
          </li>
          <li>
            <span className="ecosystem-tag dart">Pub</span>
            pubspec.lock
          </li>
          <li>
            <span className="ecosystem-tag elixir">Hex</span>
            mix.lock
          </li>
          <li>
            <span className="ecosystem-tag cocoapods">CocoaPods</span>
            Podfile.lock
          </li>
          <li>
            <span className="ecosystem-tag swift">SwiftPM</span>
            Package.resolved
          </li>
        </ul>
      </div>
    </div>
  );
}
