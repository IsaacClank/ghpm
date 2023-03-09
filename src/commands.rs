use crate::{args, github, package::Package, utils};
use std::{env::temp_dir, io};

pub fn search(args: args::SearchArgs) {
    let search_result = github::search_repo(&args.query, args.limit, args.offset);

    println!(
        "{}",
        if args.short {
            search_result.to_string_short()
        } else {
            search_result.to_string_long()
        }
    );
}

pub fn install(args: args::InstallArgs) {
    let package = Package::new(&args.formatted_repo_name());

    if package.is_installed() {
        println!("Package has already been installed");
        std::process::exit(0);
    }

    let latest_release = github::get_latest_release(&args.repo);
    let selected_asset_index = if latest_release.assets.len() == 1 {
        0
    } else {
        latest_release.print_download_urls();
        utils::read_line_from::<usize>(&mut io::stdin()).unwrap()
    };

    let selected_asset = latest_release.get_asset_by_index(selected_asset_index);
    github::download(&selected_asset.download_url);

    utils::extract_archive(
        temp_dir().join(&selected_asset.name).to_str().unwrap(),
        package.installation_path_as_str(),
    );

    // TODO: symlink
}
