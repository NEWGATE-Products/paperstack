import type { Category } from "../types";

interface CategoryTabsProps {
  categories: Category[];
  selectedCategory: string;
  onSelect: (categoryId: string) => void;
}

export function CategoryTabs({
  categories,
  selectedCategory,
  onSelect,
}: CategoryTabsProps) {
  return (
    <nav className="category-tabs">
      <div className="category-tabs-inner">
        {categories.map((category) => (
          <button
            key={category.id}
            className={`category-tab ${selectedCategory === category.id ? "active" : ""}`}
            onClick={() => onSelect(category.id)}
          >
            {getCategoryIcon(category.id)} {category.name}
          </button>
        ))}
      </div>
    </nav>
  );
}

function getCategoryIcon(categoryId: string): string {
  switch (categoryId) {
    case "all":
      return "📰";
    case "ai":
      return "🤖";
    case "llm":
      return "💬";
    case "code":
      return "💻";
    case "algorithm":
      return "🧮";
    case "architecture":
      return "🏗️";
    default:
      return "📄";
  }
}

