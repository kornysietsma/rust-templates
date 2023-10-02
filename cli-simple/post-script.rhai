// distinguish between this repo's .gitignore and the template .gitignore
file::delete(".gitignore");
file::rename("template_gitignore",".gitignore");