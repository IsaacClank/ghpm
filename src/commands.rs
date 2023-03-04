use crate::{args, env, github, utils};
use std::{path::PathBuf, io};

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
    let hub_path = PathBuf::new().join(env::Env::new().installation_root);

    let package_name = args.repo.split('/').collect::<Vec<&str>>().join("__");
    let package_root = hub_path.join(format!("opt/{package_name}"));

    if package_root.exists() {
        println!("Package already installed");
        std::process::exit(0);
    }

    let latest_release = github::get_latest_release(&args.repo);
    latest_release.print_download_urls();

    let selected_asset_index = utils::read_line_from::<usize>(&mut io::stdin()).unwrap();
    let download_url = latest_release.get_download_url(selected_asset_index);
    github::download(download_url);

    let archive_name = download_url.split('/').last().unwrap().to_string();
    utils::extract_archive(
        &format!("/tmp/{archive_name}"),
        package_root.to_str().unwrap(),
    );

    // TODO: symlink
}
