export default function Pagination({
  currentPage,
  totalPages,
  onPageChange,
}: {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}) {
  const pageNumbers = Array.from({ length: totalPages }, (_, i) => i + 1);
console.log(currentPage, pageNumbers)
  return (
    <div className="flex justify-center space-x-2 mt-4">
      {pageNumbers.map((page) => (
        <button
          key={page}
          onClick={() => onPageChange(page)}
          className={`px-4 py-2 rounded border border-base-border ${
            currentPage == page
              ? "bg-secondary text-foreground font-bold"
              : "bg-base-background text-foreground"
          }`}
        >
          {page}
        </button>
      ))}
    </div>
  );
}
