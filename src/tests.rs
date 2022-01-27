macro_rules! generate {
	{
		Realm::Server => {
			$($sv_name:ident: { $sv_from:literal, $sv_to:literal }),*
		},
		Realm::Client => {
			$($cl_name:ident: { $cl_from:literal, $cl_to:literal }),*
		}
	} => {
		mod server {
			$(
				#[test]
				fn $sv_name() {
					let mut src = $sv_from.trim().as_bytes().to_vec();
					crate::realms::optimise(crate::realms::Realm::Server, &mut src);

					let a = String::from_utf8_lossy(&src);
					let b = $sv_to.trim();
					if src.as_slice() != b.as_bytes() {
						panic!("Mismatch\n\n== Left ==\n{}\n\n== Right ==\n{}", a, b);
					}
				}
			),*
		}
		mod client {
			$(
				#[test]
				fn $cl_name() {
					let mut src = $cl_from.trim().as_bytes().to_vec();
					crate::realms::optimise(crate::realms::Realm::Client, &mut src);

					let a = String::from_utf8_lossy(&src);
					let b = $cl_to.trim();
					if src.as_slice() != b.as_bytes() {
						panic!("Mismatch\n\n== Left ==\n{}\n\n== Right ==\n{}", a, b);
					}
				}
			),*
		}
	};
}

generate! {
	Realm::Server => {
		server_and_client: {
			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SER\"VER"
			local _ = "SER\\\"VER"
			local _ = 'SER\'VER'
			local _ = [[SERVER]]
			local _ = [[SER]\]VER]]
			local _ = [[SER\]\]VER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. SERVER .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. CLIENT .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if SERVER and not lol or not hi and SERVER and CLIENT then

			end
			"#,

			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SER\"VER"
			local _ = "SER\\\"VER"
			local _ = 'SER\'VER'
			local _ = [[SERVER]]
			local _ = [[SER]\]VER]]
			local _ = [[SER\]\]VER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. true   .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. false  .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if true   and not lol or not hi and true   and false  then

			end
			"#
		}
	},

	Realm::Client => {
		server_and_client: {
			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SERVER\"SERVER"
			local _ = "SERVER\\\"SERVER"
			local _ = 'SERVER\'SERVER'
			local _ = [[SERVER]]
			local _ = [[SERVER]\]SERVER]]
			local _ = [[SERVER\]\]SERVER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. SERVER .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. CLIENT .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if SERVER and not lol or not hi and SERVER and CLIENT then

			end
			"#,

			r#"
			-- SERVER
			local _ = "SERVER CLIENT"
			local _ = "SERVER\"SERVER"
			local _ = "SERVER\\\"SERVER"
			local _ = 'SERVER\'SERVER'
			local _ = [[SERVER]]
			local _ = [[SERVER]\]SERVER]]
			local _ = [[SERVER\]\]SERVER]]
			local _ = [[SERVER CLIENT
			SERVER]]
			local _ = "" .. false  .. ""

			-- CLIENT
			local _ = "CLIENT SERVER"
			local _ = "CLIENT\"CLIENT"
			local _ = "CLIENT\\\"CLIENT"
			local _ = 'CLIENT\'CLIENT'
			local _ = [[CLIENT]]
			local _ = [[CLIENT]\]CLIENT]]
			local _ = [[CLIENT\]\]CLIENT]]
			local _ = [[CLIENT SERVER
			CLIENT]]
			local _ = "" .. true   .. ""

			--[[
				multiline comment
				SERVER
				CLIENT
			]]

			--[=[
				multiline comment
				SERVER
				CLIENT
			]=]

			--[==========[
				multiline comment
				SERVER
				CLIENT
			]==========]

			if false  and not lol or not hi and false  and true   then

			end
			"#
		}
	}
}