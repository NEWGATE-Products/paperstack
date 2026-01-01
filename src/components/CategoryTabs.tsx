import type { Category } from "../types";
import { IconNews, IconRobot, IconChat, IconComputer, IconCalculator, IconBuilding, IconDocument } from "./icons";

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
            <CategoryIcon categoryId={category.id} /> {category.name}
          </button>
        ))}
      </div>
    </nav>
  );
}

function CategoryIcon({ categoryId }: { categoryId: string }) {
  switch (categoryId) {
    case "all":
      return <IconNews size={16} />;
    case "ai":
      return <IconRobot size={16} />;
    case "llm":
      return <IconChat size={16} />;
    case "code":
      return <IconComputer size={16} />;
    case "algorithm":
      return <IconCalculator size={16} />;
    case "architecture":
      return <IconBuilding size={16} />;
    default:
      return <IconDocument size={16} />;
  }
}

