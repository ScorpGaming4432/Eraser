# Eraser
Oprogramowanie do edytowania plików Markdown napisane w języku Rust.

# Przewodnik konfiguracji projektu w Rust  

## 1. Instalacja kompilatora Rust i narzędzi  
Postępuj zgodnie z instrukcją na https://www.rust-lang.org/tools/install  

### Weryfikacja instalacji  
```bash  
rustc --version  
cargo --version  
rustup --version  
```  
> Zweryfikuj wersję dla każdego narzędzia.

---  

## 2. Kompilacja projektu z GitHub  
1. Sklonuj repozytorium:  
```bash  
git clone https://github.com/ScorpGaming4432/Eraser.git  
cd projectname  
```  
> Lub pobierz plik `.zip` z tego samego linku

2. Zbuduj projekt:  
```bash  
cargo build --release  
```  
- To polecenie pobierze zależności i skompiluje zoptymalizowane pliki binarne  

3. Uruchom projekt:  
```bash  
cargo run --release  
```  
- Lub znajdź plik wykonywalny w `target/release/Eraser`  

---  

## Rozwiązywanie problemów  
- **Błędy uprawnień**: Dodaj `sudo` przed poleceniami w systemach Linux/macOS  
- **Problemy z proxy**: Ustaw zmienne środowiskowe `http_proxy` i `https_proxy`  
- **Przestarzała wersja Rust**: Zaktualizuj za pomocą `rustup update`  
- **Błędy kompilatora**: Upewnij się, że masz zainstalowane `build-essential` (Linux) lub Xcode (macOS)  

## Kluczowe informacje:  
1. Instalator automatycznie zawiera:  
   - `rustc` (kompilator)  
   - `cargo` (menedżer pakietów)  
   - `rustup` (menedżer toolchain)  

2. Kompilacja projektu obejmuje:  
   - Rozwiązywanie zależności  
   - Kompilację przyrostową  
   - Generowanie plików binarnych  

3. Flaga `--release` tworzy zoptymalizowane buildy (wolniejsza kompilacja, ale szybsze wykonanie)  

4. Możesz odinstalować Rust w dowolnym momencie za pomocą:  
```bash  
rustup self uninstall  
```