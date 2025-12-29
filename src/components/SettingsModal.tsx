import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-shell";
import type { Settings } from "../types";

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  settings: Settings;
  onSave: (apiKey: string) => Promise<boolean>;
}

export function SettingsModal({ isOpen, onClose, settings, onSave }: SettingsModalProps) {
  const [apiKey, setApiKey] = useState("");
  const [isSaving, setIsSaving] = useState(false);
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);

  useEffect(() => {
    if (isOpen) {
      setApiKey("");
      setMessage(null);
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleSave = async () => {
    setIsSaving(true);
    setMessage(null);
    
    const success = await onSave(apiKey);
    
    if (success) {
      setMessage({ type: "success", text: "設定を保存しました" });
      setApiKey("");
      setTimeout(() => {
        onClose();
      }, 1000);
    } else {
      setMessage({ type: "error", text: "保存に失敗しました" });
    }
    
    setIsSaving(false);
  };

  const handleClear = async () => {
    setIsSaving(true);
    setMessage(null);
    
    const success = await onSave("");
    
    if (success) {
      setMessage({ type: "success", text: "APIキーを削除しました" });
    } else {
      setMessage({ type: "error", text: "削除に失敗しました" });
    }
    
    setIsSaving(false);
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>⚙️ 設定</h2>
          <button className="modal-close" onClick={onClose}>✕</button>
        </div>
        
        <div className="modal-body">
          <div className="settings-section">
            <h3>Groq API キー</h3>
            <p className="settings-description">
              日本語要約の生成に使用します。
              <a 
                href="#"
                className="settings-link"
                onClick={(e) => {
                  e.preventDefault();
                  open("https://console.groq.com/keys");
                }}
              >
                Groq Consoleでキーを取得 →
              </a>
            </p>
            
            <div className="settings-status">
              {settings.hasGroqApiKey ? (
                <span className="status-badge status-active">
                  ✓ 設定済み {settings.groqApiKey && `(${settings.groqApiKey})`}
                </span>
              ) : (
                <span className="status-badge status-inactive">
                  ✗ 未設定
                </span>
              )}
            </div>
            
            <div className="settings-input-group">
              <input
                type="password"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                placeholder="gsk_xxxxxxxxxxxxxxxx"
                className="settings-input"
                disabled={isSaving}
              />
              <button
                className="btn btn-save"
                onClick={handleSave}
                disabled={isSaving || !apiKey.trim()}
              >
                {isSaving ? "保存中..." : "保存"}
              </button>
            </div>
            
            {settings.hasGroqApiKey && (
              <button
                className="btn btn-danger"
                onClick={handleClear}
                disabled={isSaving}
              >
                🗑️ APIキーを削除
              </button>
            )}
            
            {message && (
              <div className={`settings-message ${message.type}`}>
                {message.text}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

