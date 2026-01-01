import { useEffect, useCallback, useState } from "react";
import { Header } from "./components/Header";
import { CategoryTabs } from "./components/CategoryTabs";
import { PaperList } from "./components/PaperList";
import { SettingsModal } from "./components/SettingsModal";
import { RfcList } from "./components/rfc";
import { usePapers } from "./hooks/usePapers";
import { useCategories } from "./hooks/useCategories";
import { useSettings } from "./hooks/useSettings";
import "./styles/index.css";

type MainTab = "papers" | "rfc";

function App() {
  const { papers, loading, error, getPapers, fetchPapers, generateSummary } =
    usePapers();
  const { categories, selectedCategory, setSelectedCategory } = useCategories();
  const { settings, saveGroqApiKey, refreshSettings } = useSettings();
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [mainTab, setMainTab] = useState<MainTab>("papers");

  // Load papers on mount and category change
  useEffect(() => {
    if (mainTab === "papers") {
      const category =
        selectedCategory === "all" ? undefined : getCategoryName(selectedCategory, categories);
      getPapers(category, 50);
    }
  }, [selectedCategory, getPapers, categories, mainTab]);

  const handleRefresh = useCallback(() => {
    // Get tasks for selected category
    const category = categories.find((c) => c.id === selectedCategory);
    const tasks = category?.tasks || [];
    fetchPapers(tasks);
  }, [selectedCategory, categories, fetchPapers]);

  const handleCategorySelect = (categoryId: string) => {
    setSelectedCategory(categoryId);
  };

  const handleSaveSettings = async (apiKey: string) => {
    const success = await saveGroqApiKey(apiKey);
    if (success) {
      refreshSettings();
    }
    return success;
  };

  return (
    <div className="app">
      <Header 
        onRefresh={mainTab === "papers" ? handleRefresh : undefined} 
        onOpenSettings={() => setIsSettingsOpen(true)}
        isLoading={loading === "loading"} 
        hasApiKey={settings.hasGroqApiKey}
      />
      
      {/* Main Tab Navigation */}
      <div className="main-tab-nav">
        <button
          className={`main-tab ${mainTab === "papers" ? "active" : ""}`}
          onClick={() => setMainTab("papers")}
        >
          📚 論文
        </button>
        <button
          className={`main-tab ${mainTab === "rfc" ? "active" : ""}`}
          onClick={() => setMainTab("rfc")}
        >
          📄 RFC
        </button>
      </div>

      {/* Papers Tab */}
      {mainTab === "papers" && (
        <>
          <CategoryTabs
            categories={categories}
            selectedCategory={selectedCategory}
            onSelect={handleCategorySelect}
          />
          <main className="main">
            <PaperList
              papers={papers}
              loading={loading}
              error={error}
              onGenerateSummary={generateSummary}
              onRefresh={handleRefresh}
            />
          </main>
        </>
      )}

      {/* RFC Tab */}
      {mainTab === "rfc" && (
        <main className="main rfc-main">
          <RfcList />
        </main>
      )}

      <SettingsModal
        isOpen={isSettingsOpen}
        onClose={() => setIsSettingsOpen(false)}
        settings={settings}
        onSave={handleSaveSettings}
      />
    </div>
  );
}

function getCategoryName(categoryId: string, categories: { id: string; name: string }[]): string | undefined {
  const category = categories.find((c) => c.id === categoryId);
  return category?.name;
}

export default App;
